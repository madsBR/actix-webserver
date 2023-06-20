import { goods } from "./objects.js";


export function toggleOption(optionOn,optionOff, dropdown) {
	const options = dropdown.options;
    for (let i = 0; i < options.length; i++) {
        if (options[i].textContent === optionOff && optionOff != 'none') {
            options[i].disabled = true;
        } else if (options[i].textContent === optionOn) {
            options[i].disabled = false;
        }
    }
}

export function clearRow(rows){
    for(row_ind = 0; row_ind < rows.length;row_ind++){
        rows[row_ind].style.backgroundColor = goods[0].color;
        document.getElementById('bidInput' + row_index ).value = "";
        dropdown = rows[row_ind].querySelector('select');
        dropdown.value = dropdown.options[0];
        dropdown.dispatchEvent(new Event('change'));
    }   
}


export function createRow(row_index,Choices) {
    const row = document.createElement('div');
    row.className = 'row';    
    const textField = document.createElement('input');
    textField.type = 'text';
    textField.id = 'bidInput' + row_index;
    textField.addEventListener("input", function(event) {
        event.target.value = event.target.value.replace(/\D/g, "").slice(0,30);
    })

    const dropdown = document.createElement('select');
    dropdown.dataset.index = row_index
    dropdown.addEventListener('change', function() {
        row.style.backgroundColor = this.value;            
        const selectedOption = dropdown.options[dropdown.selectedIndex].textContent;
        const prevOpt = Choices[dropdown.dataset.index];
        Choices[dropdown.dataset.index] = selectedOption;
        const rows = rowContainer.getElementsByClassName('row');
        for (let i = 0; i < rows.length; i++) {
          const dropdown = rows[i].querySelector('select');
          toggleOption(prevOpt,selectedOption,dropdown);              
        }
    });
    for (let good of goods) {
      const optionElement = document.createElement('option');
      optionElement.value = good.color;
      optionElement.textContent = good.name;
      optionElement.dataset.good_id = good.id;
      dropdown.appendChild(optionElement);
    };
    row.appendChild(textField); row.appendChild(dropdown); rowContainer.appendChild(row);
}

export function validateInput(input) {
    var regex = /^[a-zA-Z]+$/;
    var value = input.value;
  
    if (!regex.test(value)) {
      document.getElementById("error-msg").textContent = "Only alphabetic characters are allowed.";
    } else {
      document.getElementById("error-msg").textContent = "";
    }
  }
  

export function colorCodeToInteger(colorCode) {
    colorCode = colorCode.replace('#', '');
    const colorInteger = parseInt(colorCode, 16);
    
    return colorInteger;
  }
  

export function integerToColorCode(colorInteger) {
    const colorCode = '#' + colorInteger.toString(16).padStart(6, '0');
    return colorCode;
}

export function getPushBackUrl(){
    const currentUrl = window.location.href;
    if (currentUrl.slice(-1) == "/")
    {
        return currentUrl + "submit_bids"
    } else{
        return currentUrl + "/" + "submit_bids"
    }
}
window.validateInput = validateInput;