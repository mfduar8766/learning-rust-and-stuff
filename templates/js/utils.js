import SearchDropDown from './components/SearchDropDown';

// /** @typedef {{ element: HTMLElement, hasListener: boolean; domEvent: string }} THtmlElement */
// /** @type {THtmlElement} */

/**
 * @typedef {Object} Components
 * @property {string} SEARCH_DROP_DOWN - custom search drop down component
 */
/** @type {Components} */
export const COMPONENTS = {
  SEARCH_DROP_DOWN: 'search-drop-down-component',
};

/**
 * @enum {string}
 */
export const ELEMENT_TAGS = {
  DashBoard: 'dash-board',
  FilterSearch: 'filter-search',
  SearchDropDownComponent: 'search-drop-down',
};

/**
 * @type {Map<string, HTMLElement>}
 */
export const ELEMENTS = new Map()
  .set(ELEMENT_TAGS.DashBoard, getElementByID(ELEMENT_TAGS.DashBoard))
  .set(ELEMENT_TAGS.FilterSearch, getElementByID(ELEMENT_TAGS.FilterSearch))
  .set(
    ELEMENT_TAGS.SearchDropDownComponent,
    getElementByID(ELEMENT_TAGS.SearchDropDownComponent)
  );

/**
 *
 * @param {string} id
 * @returns {HTMLElement}
 */
export function getElementByID(id) {
  return document.getElementById(id);
}

export function toggleDropDown(e) {
  if (ELEMENTS.has(ELEMENT_TAGS.SearchDropDownComponent)) {
    // @ts-ignore
    ELEMENTS.get(ELEMENT_TAGS.SearchDropDownComponent).toggleDropDown();
  }
}

export function createSearchDropDownComponent() {
  customElements.define(COMPONENTS.SEARCH_DROP_DOWN, SearchDropDown);
}

export function handleRemoveElements() {
  if (ELEMENTS.size > 0) {
    for (let [key, element] of ELEMENTS.entries()) {
      if (key.length && element) {
        element.remove();
        ELEMENTS.delete(key);
      }
    }
  }
}
