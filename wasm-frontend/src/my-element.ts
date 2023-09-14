import {LitElement, PropertyValues, css, html} from "lit";
import {customElement, state} from "lit/decorators.js";
import {Table, create_table} from "../../pkg/wasm_rs";
import {memory} from "../../pkg/wasm_rs_bg.wasm";
import {observe} from "@tronicboy/lit-observe-directive";
import {queryFromEvent} from "@tronicboy/lit-from-event";
import {
  Observable,
  OperatorFunction,
  Subject,
  debounceTime,
  filter,
  map,
  shareReplay,
  startWith,
  switchMap,
  takeUntil,
} from "rxjs";
import {styleMap} from "lit/directives/style-map.js";

/**
 * An example element.
 *
 * @slot - This element has a slot
 * @csspart button - The button
 */
@customElement("my-element")
export class MyElement extends LitElement {
  @queryFromEvent("input#size", "input", {returnElementRef: true}) sizeInput$!: Observable<HTMLInputElement>;
  private canvasSquareSize$ = this.sizeInput$.pipe(
    map(el => el.value),
    map(Number),
    filter(num => !isNaN(num)),
    startWith(100)
  );
  private initChange$ = new Subject<[number, number]>();
  private mouseup$ = new Subject<void>();
  private mousedown$ = new Subject<void>();
  private changes$ = this.mousedown$.pipe(switchMap(() => this.initChange$.pipe(takeUntil(this.mouseup$))));

  private universe$ = this.canvasSquareSize$.pipe(
    map(size => create_table(new Uint8Array(size ** 2).fill(0), size, size)),
    switchMap(table =>
      this.changes$.pipe(
        map(([i, val]) => {
          table.set(i, val);
          return table;
        }),
        debounceTime(10),
        startWith(table)
      )
    ),
    this.tick(200),
    shareReplay(1)
  );
  private ticks$ = this.universe$.pipe(map(([_, ticks]) => ticks));
  private table$ = this.universe$.pipe(map(([table]) => table));

  @state() ticks = 0;

  tick(interval = 1000): OperatorFunction<Table, [Table, number]> {
    return source =>
      new Observable<[Table, number]>(observer => {
        let ticks = 0;
        let table: Table;

        let sub = source.subscribe({
          next: sourceTable => {
            table ??= sourceTable;

            observer.next([table, ticks]);
          },
          complete: () => observer.complete(),
        });

        let timer = setInterval(() => {
          if (!table) return;

          if (table.is_alive()) {
            table.tick();
            ticks += 1;

            observer.next([table, ticks]);
          }
        }, interval);

        return () => {
          clearInterval(timer);
          sub.unsubscribe();
        };
      });
  }

  render() {
    return html`
      <label for="size">Table Size:</label>
      <input type="number" min="3" .value=${observe(this.canvasSquareSize$.pipe(map(String)))} id="size" />
      <label for="ticks">Ticks:</label>
      <input type="number" readonly .value=${observe(this.ticks$.pipe(map(String)))} />
      <canvas></canvas>
      <table>
        <tbody
          @mouseup=${() => this.mouseup$.next()}
          @mousedown=${() => this.mousedown$.next()}
          @mouseleave=${() => this.mouseup$.next()}
        >
          ${observe(
            this.table$.pipe(
              map(universe => {
                const width = universe.width();
                const height = universe.height();

                const cellPtr = universe.cells();
                const cells = new Uint8Array(memory.buffer, cellPtr, width * height);

                let rows: ReturnType<typeof html>[] = [];

                for (let row = 0; row < height; row++) {
                  rows.push(
                    html`<tr>
                      ${new Array(width).fill(0).map((_, i) => {
                        const index = row * width + i;
                        const value = cells[index];
                        return html`<td
                          style=${styleMap({
                            "background-color": value === 1 ? "black" : "white",
                          })}
                          @mouseover=${() => {
                            if (index > 1) {
                              this.initChange$.next([index - 1, 1]);
                            }
                            this.initChange$.next([index, 1]);
                            if (index < width * height - 1) {
                              this.initChange$.next([index + 1, 1]);
                            }
                          }}
                        ></td>`;
                      })}
                    </tr>`
                  );
                }

                return rows;
              })
            ),
            {
              useViewTransitions: true,
            }
          )}
        </tbody>
      </table>
    `;
  }

  static styles = css`
    :host {
      max-width: 1280px;
      margin: 0 auto;
      padding: 2rem;
      text-align: center;
      user-select: none;
    }

    table {
      table-layout: fixed;
      width: 100%;
      height: 80%;
      max-height: 80vh;
      aspect-ratio: 1 / 1;
      padding: 0;
      border-collapse: collapse;
      border: 1px solid lightgray;
    }

    td {
      border: none;
      cursor: pointer;
      width: 1px;
      height: 1px;
      padding: 0;
    }
    canvas {
      margin: 1rem auto;
    }
  `;

  override update(changed: PropertyValues) {
    const update = super.update.bind(this, changed);

    if (!("startViewTransition" in document)) {
      update();
      return;
    }

    // TypeScriptはまだstartViewTransitionを知らないようです
    (document as any).startViewTransition(update);
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "my-element": MyElement;
  }
}
