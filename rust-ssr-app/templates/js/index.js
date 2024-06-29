import {
  ELEMENT_TAGS,
  createComponent,
  toggleDropDown,
  ELEMENTS,
  COMPONENTS,
} from './utils';

document.body.addEventListener('htmx:beforeOnLoad', (e) => {
  console.log('BEFORE-ON-LOAD-TYPE', typeof e);
  console.log('BEFORE-ON-LOAD-E', e);
});

document.addEventListener('htmx:configRequest', (event) => {
  console.log('CONFIG-EVENT', event);
  console.log('CONFIG-TYPE', typeof event);
  // event.detail.path = `/select_block_id${event.detail.path}`
});

document.addEventListener('htmx:beforeSwap', (e) => {
  console.log('BEFORE-SWAP-TYPE', typeof e);
  console.log('BEFORE-SWAP-EVT', e);
});

document.addEventListener('htmx:responseError', (e) => {
  console.log('RESPONSE-ERROR-TYPE', typeof e);
  console.log('RESPONSE-ERROR-EVT', e);
});

document.body.addEventListener('htmx:responseError', (e) => {
  console.log('RESPONSE-ERROR-BODY-TYPE', typeof e);
  console.log('RESPONSE-ERROR-BODY-EVT', e);
});

document.addEventListener('htmx:afterRequest', (e) => {
  console.log('REQ-ERROR-TYPE', typeof e);
  console.log('REQ-ERROR-EVT', e);
});

// document.body.addEventListener('htmx:beforeSwap', function(evt) {
//   console.log('BEFORE-SW-TYPE', typeof evt);
//   console.log('BEFORE-SWAP-EVT', evt);
//   // Allow 422 and 400 responses to swap
//   // We treat these as form validation errors
//   // if (evt.detail.xhr.status === 422 || evt.detail.xhr.status === 400) {
//   //   evt.detail.shouldSwap = true;
//   //   evt.detail.isError = false;
//   // }
// });

// document.body.addEventListener('htmx:responseError', function (evt) {
//   const errorTarget = document.getElementById("htmx-alert")
//   console.log('TYPE-E', typeof evt);
//   console.log('EVT', evt);
//   // if (evt.detail.successful) {
//   //     // Successful request, clear out alert
//   //     errorTarget.setAttribute("hidden", "true")
//   //     errorTarget.innerText = "";
//   // } else if (evt.detail.failed && evt.detail.xhr) {
//   //     // Server error with response contents, equivalent to htmx:responseError
//   //     console.warn("Server error", evt.detail)
//   //     const xhr = evt.detail.xhr;
//   //     errorTarget.innerText = `Unexpected server error: ${xhr.status} - ${xhr.statusText}`;
//   //     errorTarget.removeAttribute("hidden");
//   // } else {
//   //     // Unspecified failure, usually caused by network error
//   //     console.error("Unexpected htmx error", evt.detail)
//   //     errorTarget.innerText = "Unexpected error, check your connection and try to refresh the page.";
//   //     errorTarget.removeAttribute("hidden");
//   // }
// });

// document.getElementById('log-in-form').addEventListener('htmx:responseError', function(evt) {
//   console.log('EVT', evt);
//   console.log('TYPE', typeof evt);
//   document.getElementById('error-message').innerHTML = "foo";
// });

// document.addEventListener('htmx:afterRequest', (event) => {
//   console.log('EVENT', event);
//   // @ts-ignore
//   if (event.detail.target.id === 'app') {
//     window.location.href = '/dash-board';
//   }
// });

// if (document.getElementById('log-in-form')) {
//   handleRemoveElements();
// }

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
}
