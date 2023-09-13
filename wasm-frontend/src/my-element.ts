import {LitElement, PropertyValueMap, css, html} from "lit";
import {customElement, state} from "lit/decorators.js";
import {Table, create_table} from "../../pkg/wasm_rs";
import {memory} from "../../pkg/wasm_rs_bg.wasm";
import {observe} from "@tronicboy/lit-observe-directive";
import {
  Observable,
  OperatorFunction,
  Subject,
  buffer,
  debounceTime,
  map,
  shareReplay,
  startWith,
  switchMap,
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
  private canvasSquareSize$ = new Subject<number>();
  private initChange$ = new Subject<[number, number]>();
  private mouseup$ = new Subject<void>();
  private mousedown$ = new Subject<void>();

  private universe$ = this.canvasSquareSize$.pipe(
    map(size => create_table(new Uint8Array(size ** 2).fill(0), size, size)),
    this.cacheLastTable(this.initChange$),
    shareReplay(1)
  );
  private tick$ = this.universe$.pipe(this.tick());

  @state() ticks = 0;

  protected firstUpdated(_changedProperties: PropertyValueMap<any> | Map<PropertyKey, unknown>): void {
    this.canvasSquareSize$.next(100);

    this.tick$.subscribe(([_, ticks]) => {
      this.ticks = ticks;
    });
  }

  cacheLastTable(changes$: Observable<[number, number]>): OperatorFunction<Table, Table> {
    return source => {
      let lastTable: Table;
      let lastArrayBuff: Uint8Array | undefined;

      return source.pipe(
        switchMap(initTable => {
          lastTable = initTable;

          return this.mousedown$.pipe(
            switchMap(() => changes$.pipe(buffer(this.mouseup$))),
            map(changes => {
              const cellPtr = lastTable.cells();
              const cells = new Uint8Array(memory.buffer, cellPtr, initTable.width() * initTable.height());

              lastArrayBuff ??= structuredClone(cells);

              for (const [i, val] of changes) {
                lastArrayBuff[i] = val;
              }

              return lastArrayBuff;
            }),
            map(values => {
              let lastTable = create_table(values, initTable.width(), initTable.height());

              return lastTable;
            }),
            startWith(initTable)
          );
        })
      );
    };
  }

  tick(): OperatorFunction<Table, [Table, number]> {
    return source =>
      source.pipe(
        switchMap(
          table =>
            new Observable<[Table, number]>(observer => {
              let ticks = 0;
              let timer = setInterval(() => {
                table.tick();
                ticks += 1;

                observer.next([table, ticks]);

                if (ticks % 5 === 0) {
                  const cells = getCellsFromTable(table);
                  if (cells.every(cell => cell === 0)) {
                    observer.complete();
                  }
                }
              }, 100);

              return () => clearInterval(timer);
            })
        )
      );
  }

  render() {
    return html`
      <label for="size">Table Size:</label>
      <input type="number" min="3" value="100" id="size" />
      <label for="ticks">Ticks:</label>
      <input type="number" readonly .value=${String(this.ticks)} />
      <canvas></canvas>
      <table>
        <tbody
          @mouseup=${() => this.mouseup$.next()}
          @mousedown=${() => this.mousedown$.next()}
        >
          ${observe(
            this.universe$.pipe(
              map(universe => {
                console.log(universe);
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
                            this.initChange$.next([index, 1]);
                          }}
                        ></td>`;
                      })}
                    </tr>`
                  );
                }

                return rows;
              })
            )
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
}

declare global {
  interface HTMLElementTagNameMap {
    "my-element": MyElement;
  }
}

function getCellsFromTable(table: Table): Uint8Array {
  const cellPtr = table.cells();
  return new Uint8Array(memory.buffer, cellPtr, table.width() * table.height());
}
