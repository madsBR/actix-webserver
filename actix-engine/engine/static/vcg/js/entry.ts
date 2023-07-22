import { toggleOption, validateInput, colorCodeToInteger, integerToColorCode, getPushBackUrl, createRow, clearRow, has_null_good_selected } from './helpers.js';
import { GoodObj, PlayerObj,createPlayerObj,createGoodObj, goods, NULL_GOOD } from './objects.js';
declare global {
  interface Window {
    validateInput: (input: HTMLInputElement) => boolean;
  }
}
interface Content {
  id: number;
  player_nr: number;
  pls: PlayerObj[];
  goods: GoodObj[];
  bid_pairings: [number, number, number][];
}

document.addEventListener('DOMContentLoaded', function () {
  let nr_submitted: number = 0;
  const selectN = document.getElementById('selectN') as HTMLSelectElement;
  const rowContainer = document.getElementById('rowContainer') as HTMLDivElement;
  let content: Content = {
    id: Math.floor(Math.random() * 9007199254740990) + 1,
    player_nr: parseInt(selectN.value),
    pls: [],
    goods: goods,
    bid_pairings: [],
  };
  let Choices: string[] | undefined;

  selectN.addEventListener('change', function () {
    let confirmed: boolean;
    if (nr_submitted > 0) {
      confirmed = confirm('Are you sure you want to change the number of players? Bids will be reset');
    } else {
      confirmed = true;
    }
    if (confirmed) {
      this.dataset.previousValue = this.value;
      const selectedN = parseInt(this.value);
      Choices = Array(selectedN).fill('none');
      rowContainer.innerHTML = '';
      for (let i = 0; i < selectedN; i++) {
        createRow(i,rowContainer, Choices);
      }
      reset();
    } else {
      this.value = this.dataset.previousValue || 'default-value'; // Use a default value here
    }
  });

  const submitBtn = document.getElementById('submitBids') as HTMLButtonElement;
  submitBtn.onclick = function () {
    if (nr_submitted === 0) {
      content.player_nr = parseInt(selectN.value);
    }
    const nameInput = document.getElementById('name') as HTMLInputElement;
    // checks
    if (!validateInput(nameInput)) {
      document.getElementById('submit-error-msg')!.textContent = 'name is invalid';
    } else {
      document.getElementById('submit-error-msg')!.textContent = '';
      content.pls.push(createPlayerObj(nr_submitted, nameInput.value));
      const rows = rowContainer.getElementsByClassName('row');
      for (let i = 0; i < parseInt(selectN.value); i++) {
        const row = rows[i] as HTMLDivElement;
        const dropdown = row.querySelector('select') as HTMLSelectElement;
        if (!has_null_good_selected(dropdown)) {
          const good_id = parseInt(dropdown.options[dropdown.selectedIndex].dataset.good_id!);
          console.log('putting in. good id is ' + good_id);
          const bid = parseInt((document.getElementById('bidInput' + i) as HTMLInputElement).value);
          content.bid_pairings.push([nr_submitted, good_id, bid]);
        }
      }
      // Send a POST request to the current page's route
      nr_submitted += 1;
      console.log('players submitted are ' + nr_submitted + ' content is : ' + JSON.stringify(content));

      if (nr_submitted === parseInt(selectN.value)) {
        console.log(getPushBackUrl());
        fetch(getPushBackUrl(), {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(content),
        })
          .then(response => response.text())
          .then(html => {
            console.log('RECIEVED RESULT, NOW PUTTING IT IN');
            document.body.innerHTML = html;
            // Process the response from the server
          })
          .catch(error => {
            console.error('Error:', error);
            // Handle errors
          });
      } else {
        clearRow(rowContainer.getElementsByClassName('row') as HTMLCollectionOf<HTMLDivElement>);
        (document.getElementById('name') as HTMLInputElement).value = '';
      }

      //console.log("content bid pairings:" + content.bid_pairings.toString());
    }
  };
  selectN.dispatchEvent(new Event('change'));

  function reset() {
    nr_submitted = 0;
    content = {
      id: Math.floor(Math.random() * 9007199254740990) + 1,
      player_nr: parseInt(selectN.value),
      pls: [],
      goods: goods,
      bid_pairings: [],
    };
    clearRow(rowContainer.getElementsByClassName('row') as HTMLCollectionOf<HTMLDivElement>);
    (document.getElementById('name') as HTMLInputElement).value = '';
  }
});
