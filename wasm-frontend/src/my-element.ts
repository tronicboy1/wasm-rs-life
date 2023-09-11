import { LitElement, css, html } from "lit";
import { customElement, property } from "lit/decorators.js";
import { greet } from "../../pkg/wasm_rs";
import { observe } from "@tronicboy/lit-observe-directive";
import { Observable, map, timer } from "rxjs";

/**
 * An example element.
 *
 * @slot - This element has a slot
 * @csspart button - The button
 */
@customElement("my-element")
export class MyElement extends LitElement {
  private init: boolean[][] = [
    [true, false, false, false, false],
    [true, false, false, false, false],
    [true, false, false, false, false],
    [true, false, false, false, false],
    [true, false, false, false, false],
    [true, false, false, false, false],
  ];
  private universe$ = new Observable<boolean[][]>((observer) => {
    let state: boolean[][] = this.init;

    const t = setTimeout(() => {
      observer.next(state);
    }, 1000);

    return () => clearTimeout(t);
  });

  connectedCallback(): void {
    super.connectedCallback();
  }

  render() {
    return html`
      <table>
        <tbody>
          ${observe(
            this.universe$.pipe(
              map((uni) =>
                uni.map(
                  (row) =>
                    html`<tr>
                      ${row.map((col) => col ? "*" : " ").map((col) => html`<td>${col}</td>`)}
                    </tr>`,
                ),
              ),
            ),
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
      border: 1px solid;
      table-layout: fixed;
      width: 100%;
      height: 80%
    }
  `;
}

declare global {
  interface HTMLElementTagNameMap {
    "my-element": MyElement;
  }
}
