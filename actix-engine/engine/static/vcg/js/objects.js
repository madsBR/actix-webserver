export function GoodObj(id, name, color) {
    this.id = id;
    this.name = name;
    this.color = color;
}


export function PlayerObj(id, name) {
  this.id = id;
  this.name = name;
}


export const NULL_GOOD = new GoodObj(6,'none','#FFFFFF')

export const goods = [
    new GoodObj(0,'Firaks','#808B96'),
    new GoodObj(1,'Ivits','#FF0000'),
    new GoodObj(2,'Terran','#0000FF'),
    new GoodObj(3,'Xenon','#FFFF00'),
    new GoodObj(4,'Geoden','#F39c12'),
    new GoodObj(5,'Itars','#FFFFFF'),
    NULL_GOOD
  ];

export const NULL_GOOD_INDEX = goods.length-1; 
