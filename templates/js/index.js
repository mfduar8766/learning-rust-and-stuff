import {
  ELEMENT_TAGS,
  createComponent,
  toggleDropDown,
  handleRemoveElements,
  ELEMENTS,
  COMPONENTS,
} from './utils';

// document.addEventListener('htmx:afterRequest', (event) => {
//   console.log('EVENT', event);
//   // @ts-ignore
//   if (event.detail.target.id === 'app') {
//     window.location.href = '/dash-board';
//   }
// });

if (
  ELEMENTS.get(ELEMENT_TAGS.DashBoard) &&
  ELEMENTS.get(ELEMENT_TAGS.FilterSearch) &&
  ELEMENTS.get(ELEMENT_TAGS.SettingsGear)
) {
  createComponent(COMPONENTS.SEARCH_DROP_DOWN);
  createComponent(COMPONENTS.SETTINGS_DROP_DOWN);
  ELEMENTS.get(ELEMENT_TAGS.FilterSearch).addEventListener(
    'click',
    toggleDropDown
  );
  ELEMENTS.get(ELEMENT_TAGS.SettingsGear).addEventListener(
    'click',
    toggleDropDown
  );
} else {
  handleRemoveElements();
}
