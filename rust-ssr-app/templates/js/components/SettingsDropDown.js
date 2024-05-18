export default class SettingsDropDown extends HTMLElement {
  css = `
  :host {
    position: relative;
    width: 100%;
  }

  .dropdown-content {
  display: none;
  position: absolute;
  background-color: #fff;
  min-width: 100%;
  overflow: auto;
  border: 1px solid #ddd;
  z-index: 1;
  height: auto;
}

.dropdown-content a {
  color: black;
  padding: 12px 16px;
  text-decoration: none;
  display: block;
}

.dropdown-content a:hover {background-color: #ddd;}

.show {
  display: inline-block;
  width: max-content
}

.cursor {
  cursor: pointer;
}
`;

  /**
   * @type {ShadowRoot}
   */
  root = undefined;

  /**
   * @typedef {Object} ComponentData
   * @property {string} api_url - api_url: the url for the apis
   */
  /** @type {ComponentData} */
  data = {
    api_url: '',
  };

  constructor() {
    super();
    this.root = this.attachShadow({ mode: 'open' });
    this.data = { ...this.data, api_url: this.getAttribute('data') || '' };
    this.render();

    // @ts-ignore
    htmx.process(this.root.getElementById('custom-search-drop-down'));
  }

  toggleDropDown() {
    this.root
      .getElementById('custom-search-drop-down')
      .classList.toggle('show');
  }

  /**
   * @returns boolean
   */
  hasShowClassAttribute() {
    return this.root.getElementById('custom-search-drop-down').classList.contains('show');
  }

  template() {
    return `<div id="custom-search-drop-down" class="cursor dropdown-content">
    <a class="cursor">Profile</a>
    <a hx-post="${this.data.api_url}logout"
    hx-target="#dash-board"
    hx-swap="outerHTML">Log Out</a>
  </div>`;
  }

  render() {
    this.root.innerHTML = `
    <style>${this.css.trim()}</style>
    ${this.template().trim()}
  `;
  }
}
