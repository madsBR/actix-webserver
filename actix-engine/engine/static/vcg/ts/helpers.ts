import { NULL_GOOD, GoodExt, goods, NULL_GOOD_INDEX } from './objects';

export function is_null_good(good: GoodExt): boolean {
  return good === NULL_GOOD;
}

export function has_null_good_selected(dropdown: HTMLSelectElement): boolean {
  return parseInt(dropdown.options[dropdown.options.selectedIndex].dataset.good_id!) === NULL_GOOD.id;
}

export function toggleOption(optionOn: string, optionOff: string, dropdown: HTMLSelectElement): void {
  const options = dropdown.options;
  for (let i = 0; i < options.length; i++) {
    if (options[i].textContent === optionOff && optionOff !== 'none') {
      options[i].disabled = true;
    } else if (options[i].textContent === optionOn) {
      options[i].disabled = false;
    }
  }
}

export function clearRow(rows: HTMLCollectionOf<HTMLDivElement>): void {
  for (let row_ind = 0; row_ind < rows.length; row_ind++) {
    rows[row_ind].style.backgroundColor = goods[0].color.str;
    (document.getElementById('bidInput' + row_ind) as HTMLInputElement).value = "";
    const dropdown = rows[row_ind].querySelector('select') as HTMLSelectElement;
    dropdown.selectedIndex = NULL_GOOD_INDEX;
    dropdown.dispatchEvent(new Event('change'));
  }
}

export function createRow(row_index: number,rowContainer: HTMLElement, Choices: (string | undefined)[]): void {
  const row = document.createElement('div');
  row.className = 'row';
  const textField = document.createElement('input');
  textField.type = 'text';
  textField.id = 'bidInput' + row_index;
  textField.addEventListener("input", function (event) {
    const inputElement = event.target as HTMLInputElement; // Type assertion
    inputElement.value = inputElement.value.replace(/\D/g, "").slice(0, 30);
  });
  const dropdown = document.createElement('select');
  dropdown.setAttribute("id", "select" + row_index);
  dropdown.dataset.index = row_index.toString();
  for (const good of goods) {
    const optionElement = document.createElement('option');
    optionElement.value = good.color.str;
    optionElement.textContent = good.name;
    optionElement.dataset.good_id = good.id.toString();
    dropdown.appendChild(optionElement);
  }
  dropdown.addEventListener('change', function () {
    row.style.backgroundColor = this.value;
    const selectedOption = dropdown.options[dropdown.options.selectedIndex].textContent!;
    const prevOpt = Choices[parseInt(dropdown.dataset.index || '0')] || 'none';
    Choices[parseInt(dropdown.dataset.index || '+')] = selectedOption;
    const rows = rowContainer.getElementsByClassName('row');
    for (let i = 0; i < rows.length; i++) {
      const this_dropdown = rows[i].querySelector('select') as HTMLSelectElement;
      toggleOption(prevOpt, selectedOption, this_dropdown);
    }
  });
  row.appendChild(textField);
  row.appendChild(dropdown);
  rowContainer.appendChild(row);
}

export function validateInput(input: HTMLInputElement): boolean {
  const regex = /^[a-zA-Z]+$/;
  const val = input.value;
  if (!regex.test(val)) {
    document.getElementById("error-msg")!.textContent = "Only alphabetic characters are allowed.";
    return false;
  }
  if (val.replace(/\s/g, '') === "") {
    document.getElementById("error-msg")!.textContent = "can't be empty";
    return false;
  } else {
    document.getElementById("error-msg")!.textContent = "";
    return true;
  }
}

export function colorCodeToInteger(colorCode: string): number {
  colorCode = colorCode.replace('#', '');
  const colorInteger = parseInt(colorCode, 16);
  return colorInteger;
}

export function integerToColorCode(colorInteger: number): string {
  const colorCode = '#' + colorInteger.toString(16).padStart(6, '0');
  return colorCode;
}

export function getPushBackUrl(): string {
  const currentUrl = window.location.href;
  if (currentUrl.slice(-1) === "/") {
    return currentUrl + "submit_bids";
  } else {
    return currentUrl + "/" + "submit_bids";
  }
}

window.validateInput = validateInput;
