import {
  ELEMENT_TAGS,
  createSearchDropDownComponent,
  toggleDropDown,
  handleRemoveElements,
  ELEMENTS,
} from './utils';

if (
  ELEMENTS.has(ELEMENT_TAGS.DashBoard) &&
  ELEMENTS.has(ELEMENT_TAGS.FilterSearch) &&
  ELEMENTS.get(ELEMENT_TAGS.DashBoard) &&
  ELEMENTS.has(ELEMENT_TAGS.FilterSearch)
) {
  createSearchDropDownComponent();
  ELEMENTS.get(ELEMENT_TAGS.FilterSearch).addEventListener(
    'click',
    toggleDropDown
  );
} else {
  handleRemoveElements();
}
