import {
  ELEMENT_TAGS,
  createComponent,
  toggleDropDown,
  handleRemoveElements,
  ELEMENTS,
} from './utils';

if (
  ELEMENTS.has(ELEMENT_TAGS.DashBoard) &&
  ELEMENTS.has(ELEMENT_TAGS.FilterSearch) &&
  ELEMENTS.get(ELEMENT_TAGS.DashBoard) &&
  ELEMENTS.get(ELEMENT_TAGS.FilterSearch)
) {
  createComponent(ELEMENT_TAGS.SearchDropDownComponent);
  createComponent(ELEMENT_TAGS.SettingsDropDownComponent);
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
