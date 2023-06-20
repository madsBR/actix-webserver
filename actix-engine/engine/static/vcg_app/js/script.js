import { toggleOption, validateInput, colorCodeToInteger, integerToColorCode, getPushBackUrl ,createRow, clearRow} from './helpers.js';
import { GoodObj, PlayerObj, goods } from './objects.js';


document.addEventListener('DOMContentLoaded', function() {

  
  let nr_submitted = 0;
  const selectN = document.getElementById('selectN');
  const rowContainer = document.getElementById('rowContainer');
  var content = {
    id : Math.floor(Math.random() * 9007199254740990 ) + 1,
    player_nr: parseInt(selectN.value),
    pls : [],
    goods : goods,
    bid_pairings : [],
  };
  let Choices;
  
  selectN.addEventListener('change', function() {
    var confirmed;
    if ( nr_submitted >0){
      this.value = this.dataset.previousValue;
      confirmed = confirm('Are you sure you want to change number of players? Bids will be reset');
    } else{
      confirmed = true;
    }
    if(confirmed){
      this.dataset.previousValue = this.value;
      const selectedN = parseInt(this.value);
      Choices = Array(selectedN).fill("none");
      rowContainer.innerHTML = '';
      for (let i = 0; i < selectedN; i++) {
        createRow(i,Choices);                        
      }
      reset()
    }
  });
  
  const submitBtn= document.getElementById('submitBids');  
  submitBtn.onclick = function(){
    if (nr_submitted == 0){content.player_nr = parseInt(selectN.value);}
    var nameInput = document.getElementById("name");
    console.log("captured the following input" + nameInput.value);
    content.pls.push(new PlayerObj(nr_submitted,nameInput.value));
    console.log("captured the following input" + content.pls[content.pls.length-1].name);
    const rows = rowContainer.getElementsByClassName('row');
    for (let i = 0; i < selectN.value; i++) {
      const row = rows[i];
      const dropdown = row.querySelector('select');
      const good_id = dropdown.options[dropdown.selectedIndex].dataset.good_id
      if (good_id > 0){        
        const bid = document.getElementById('bidInput' + i).value;
        content.bid_pairings.push([parseInt(nr_submitted),parseInt(good_id),parseInt(bid)]);
      }
    }
            // Send a POST request to the current page's route
    nr_submitted +=1;
    console.log("players submitted are " + nr_submitted + " content is : " + JSON.stringify(content) );

    if (nr_submitted == parseInt(selectN.value)){
      console.log(getPushBackUrl());
      fetch(getPushBackUrl(), {
          method: 'POST',
          headers: {
              'Content-Type': 'application/json'
          },
          body: JSON.stringify(content)
      })
      .then(response => response.text())
      .then(html => {
        console.log("RECIEVED RESULT, NOW PUTTING IT IN")
        document.body.innerHTML = html;
          // Process the response from the server
      })
      .catch(error => {
          console.error('Error:', error);
          // Handle errors
      });
    }

    //console.log("content bid pairings:" + content.bid_pairings.toString());
  }
  selectN.dispatchEvent(new Event('change'));

  function reset(){
    nr_submitted = 0;
    content = {
      id : Math.floor(Math.random() * 9007199254740990 ) + 1,
      player_nr: parseInt(selectN.value),
      pls : [],
      goods : goods,
      bid_pairings : [],
    } 
    clearRow(rowContainer.getElementsByClassName('row'));
    nameInput.value="";
  }
  });



