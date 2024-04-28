import SearchDropDown from './components/SearchDropDown';
import SettingsDropDown from './components/SettingsDropDown';

// /** @typedef {{ element: HTMLElement, hasListener: boolean; domEvent: string }} THtmlElement */
// /** @type {THtmlElement} */

/**
 * @typedef {Object} Components
 * @property {string} SEARCH_DROP_DOWN -  search-drop-down-component custom search drop down component
 * @property {string} SETTINGS_DROP_DOWN - settings-drop-down-component custom drop down for settings
 */
/** @type {Components} */
export const COMPONENTS = {
  SEARCH_DROP_DOWN: 'search-drop-down-component',
  SETTINGS_DROP_DOWN: 'settings-drop-down-component',
};

/**
 * @enum {string}
 */
export const ELEMENT_TAGS = {
  DashBoard: 'dash-board',
  FilterSearch: 'filter-search',
  SearchDropDownComponent: 'search-drop-down',
  SettingsDropDownComponent: 'settings-drop-down',
  SettingsGear: 'settings-gear',
};

/**
 * @type {Map<string, HTMLElement>}
 */
export const ELEMENTS = new Map()
  .set(ELEMENT_TAGS.DashBoard, getElementByID(ELEMENT_TAGS.DashBoard))
  .set(ELEMENT_TAGS.FilterSearch, getElementByID(ELEMENT_TAGS.FilterSearch))
  .set(ELEMENT_TAGS.SettingsGear, getElementByID(ELEMENT_TAGS.SettingsGear));

/**
 * @type {Map<string, HTMLElement>}
 */
export const CUSTOM_ELEMENTS = new Map()
  .set(
    ELEMENT_TAGS.SettingsDropDownComponent,
    getElementByID(ELEMENT_TAGS.SettingsDropDownComponent)
  )
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

/**
 * @param {MouseEvent} e
 */
export function toggleDropDown(e) {
  if (
    CUSTOM_ELEMENTS.has(ELEMENT_TAGS.SearchDropDownComponent) &&
    // @ts-ignore
    getElementByID(e.target.id).id === ELEMENT_TAGS.FilterSearch
  ) {
    // @ts-ignore
    CUSTOM_ELEMENTS.get(ELEMENT_TAGS.SearchDropDownComponent).toggleDropDown();
  } else if (
    CUSTOM_ELEMENTS.has(ELEMENT_TAGS.SettingsDropDownComponent) &&
    // @ts-ignore
    getElementByID(e.target.id).id === ELEMENT_TAGS.SettingsGear
  ) {
    // @ts-ignore
    CUSTOM_ELEMENTS.get(
      ELEMENT_TAGS.SettingsDropDownComponent
      // @ts-ignore
    ).toggleDropDown();
  }
}

/**
 * @param {string} component
 */
export function createComponent(component) {
  switch (component) {
    case COMPONENTS.SEARCH_DROP_DOWN:
      // @ts-ignore
      customElements.define(component, SearchDropDown);
      break;
    case COMPONENTS.SETTINGS_DROP_DOWN:
      // @ts-ignore
      customElements.define(component, SettingsDropDown);
    default:
      break;
  }
}

export function handleRemoveElements() {
  console.log('HANDLE-REMOVE');
  if (CUSTOM_ELEMENTS.size > 0) {
    for (let [key, element] of CUSTOM_ELEMENTS.entries()) {
      if (key.length && element) {
        element.remove();
        ELEMENTS.delete(key);
      }
    }
  }
}
