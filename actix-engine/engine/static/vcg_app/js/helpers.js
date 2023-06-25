import { goods,NULL_GOOD,NULL_GOOD_INDEX} from "./objects.js";

function is_null_good (good){
    return(good == NULL_GOOD)
}

export function has_null_good_selected(dropdown){
    return(dropdown.options[dropdown.options.selectedIndex].dataset.good_id == NULL_GOOD.id)
}




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

    for(var row_ind = 0; row_ind < rows.length;row_ind++){
        rows[row_ind].style.backgroundColor = goods[0].color;
        document.getElementById('bidInput' + row_ind ).value = "";
        const dropdown = rows[row_ind].querySelector('select');
        dropdown.selectedIndex = NULL_GOOD_INDEX;   
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

    const dropdown = document.createElement('select',);
    dropdown.setAttribute("id","select" + row_index);
    dropdown.dataset.index = row_index
    for (let good of goods) {
        const optionElement = document.createElement('option');
        optionElement.value = good.color;
        optionElement.textContent = good.name;
        optionElement.dataset.good_id = good.id;
        dropdown.appendChild(optionElement);
    };

    dropdown.addEventListener('change', function() {
        row.style.backgroundColor = this.value;            
        const selectedOption = dropdown.options[dropdown.options.selectedIndex].textContent;
        const prevOpt = Choices[dropdown.dataset.index];
        Choices[dropdown.dataset.index] = selectedOption;
        const rows = rowContainer.getElementsByClassName('row');
        for (let i = 0; i < rows.length; i++) {
          const this_dropdown = rows[i].querySelector('select');
          toggleOption(prevOpt,selectedOption,this_dropdown);              
        }

    });
    
    row.appendChild(textField); row.appendChild(dropdown); rowContainer.appendChild(row);
}

export function validateInput(input) {
    const regex = /^[a-zA-Z]+$/;
    const val = input.value;
    if (!regex.test(val)) {
      document.getElementById("error-msg").textContent = "Only alphabetic characters are allowed.";
      return(false);
    } 

    if (val.replace(/\s/g,'') == "" ){
        document.getElementById("error-msg").textContent = "can't be empty";
        return(false);
    }
    else {
        document.getElementById("error-msg").textContent = "";
        return(true);
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