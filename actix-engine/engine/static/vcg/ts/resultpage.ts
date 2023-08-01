import { PlayerExt,GoodWPriceExt,OutputPairing } from "./objects";

const jsonString = `[
    {
      "pl": {
        "id": 1,
        "name": "John Doe"
      },
      "good_color_price": {
        "good": {
          "id": 100,
          "name": "Toy",
          "color": {
            "str": "#00AABB"
          }
        },
        "price": {
          "val": 50
        }
      }
    },
    {
      "pl": {
        "id": 2,
        "name": "Jane Smith"
      },
      "good_color_price": {
        "good": {
          "id": 200,
          "name": "Book",
          "color": {
            "str": "#AAFF11"
          }
        },
        "price": {
          "val": 30
        }
      }
    }
  ]`;
    
  // Assuming you have the parsedData array from the previous step
  
  // Interface for the row data with the required fields
  interface TableRow {
    playerName: string;
    goodName: string;
    price: number;
    goodColor: string;
  }


  function convertPlGwToTableRow(pl : PlayerExt,good_color_price : GoodWPriceExt): TableRow {
    return {
        playerName: pl.name,
        goodName: good_color_price.good.name,
        price: good_color_price.price.val,
        goodColor: good_color_price.good.color.str,
        }
    }

    function convertPlNullToTableRow(pl : PlayerExt): TableRow {
        return {
            playerName: pl.name,
            goodName: 'none',
            price: 0,
            goodColor: 'gray',
        }
    }
    
  // Function to convert OutputPairing to TableRow
  function convertToTableRow(outputPairing: OutputPairing): TableRow{
    const { pl, good_color_price } = outputPairing;
    return good_color_price ? convertPlGwToTableRow(pl,good_color_price) : convertPlNullToTableRow(pl)
  }
  
  // Function to create a row dynamically based on TableRow data with a fade-in effect
  function createRow(rowData: TableRow): HTMLTableRowElement {
    const row = document.createElement('tr');
  
    const playerNameCell = document.createElement('td');
    playerNameCell.textContent = rowData.playerName;
    row.appendChild(playerNameCell);
  
    const goodNameCell = document.createElement('td');
    goodNameCell.textContent = rowData.goodName;
    row.appendChild(goodNameCell);
  
    const priceCell = document.createElement('td');
    priceCell.textContent = rowData.price.toString();
    row.appendChild(priceCell);
  
    // Apply the color value directly to the background color of the row
    row.style.backgroundColor = rowData.goodColor;
  
    // Add the "fade-in" class to the row to trigger the fade-in effect
    setTimeout(() => {
      row.classList.add('fade-in');
    }, 50); // Use a small delay to ensure the class is added after row creation
  
    return row;
  }
  
  // Function to add a row with a delay of 0.5 seconds
  function addRowWithDelay(tableBody: HTMLTableSectionElement, data: OutputPairing[], rowIndex: number) {
    if (rowIndex >= data.length) {
      return; // Exit the function when all rows are added
    }
  
    const tableRowData = convertToTableRow(data[rowIndex]);
    const row = createRow(tableRowData);
    tableBody.appendChild(row);
  
    // Schedule the next row creation with a delay of 0.5 seconds (500 milliseconds)
    setTimeout(() => {
      addRowWithDelay(tableBody, data, rowIndex + 1);
    }, 500);
  }
  
  // Function to create rows dynamically with a delay of 0.5 seconds between each row
  function createRows(data: OutputPairing[]) {
    const tableBody : HTMLTableSectionElement | null= document.querySelector('#outputPairingTable tbody');
    if (tableBody != null){
        addRowWithDelay(tableBody, data, 0); // Start creating rows with a delay
    }
  }
  
  // Call the createRows function with the parsedData array to create rows with a delay
  