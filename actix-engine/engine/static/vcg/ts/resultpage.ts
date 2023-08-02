import { PlayerExt,GoodWPriceExt,OutputPairing, Color } from "./objects";
import { hexToRgb } from "./helpers";
export {ResultObject}

  // Assuming you have the parsedData array from the previous step
  
  

  interface ResultObject{
    html : string,
    auction_result : OutputPairing[],
    input_matrix : [number,number | null,number][]
  }

  

  // Interface for the row data with the required fields
  interface TableRow {
    playerName: string;
    goodName: string;
    price: number;
    goodColor: Color;
  }


  function convertPlGwToTableRow(pl : PlayerExt,good_color_price : GoodWPriceExt): TableRow {
    return {
        playerName: pl.name,
        goodName: good_color_price.good.name,
        price: good_color_price.price.val,
        goodColor: good_color_price.good.color,
        }
    }

    function convertPlNullToTableRow(pl : PlayerExt): TableRow {
        return {
            playerName: pl.name,
            goodName: 'none',
            price: 0,
            goodColor: {str: 'gray'},
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

    const font_col : Color = font_color_from_bg(rowData.goodColor)
    
    const playerNameCell : HTMLTableCellElement= document.createElement('td');
    playerNameCell.textContent = rowData.playerName;
    playerNameCell.style.color = font_col.str
    row.appendChild(playerNameCell);
  
    const goodNameCell : HTMLTableCellElement = document.createElement('td');
    goodNameCell.textContent = rowData.goodName;
    goodNameCell.style.color = font_col.str
    row.appendChild(goodNameCell);
  
    const priceCell : HTMLTableCellElement = document.createElement('td');
    priceCell.textContent = rowData.price.toString();
    priceCell.style.color = font_col.str
    row.appendChild(priceCell);
  
    // Apply the color value directly to the background color of the row
    row.style.backgroundColor = rowData.goodColor.str;
  
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
  function createResultRows(data: OutputPairing[]) {
    const tableBody : HTMLTableSectionElement | null= document.querySelector('#outputPairingTable tbody');
    if (tableBody != null){
        addRowWithDelay(tableBody, data, 0); // Start creating rows with a delay
    }
  }
  
  export function displayResult(res_obj : ResultObject) {
    
      document.body.innerHTML = res_obj.html;
      createResultRows(res_obj.auction_result);
      
      // Process the response from the server
    
  }

  function font_color_from_bg( bg : Color) : Color{
    const rgb : [number,number,number] = hexToRgb(bg) ?? [255,255,255];
    if ((rgb[0]*0.299 + rgb[1]*0.587 + rgb[2]*0.114) > 186){return {str : "#000000"}} else {return {str : "#FFFFFF"}}
  }
  // Call the createRows function with the parsedData array to create rows with a delay
  