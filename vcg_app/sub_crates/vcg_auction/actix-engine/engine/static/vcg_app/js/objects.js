export function GoodObj(id, name, color) {
    this.id = id;
    this.name = name;
    this.color = color;
}


export function PlayerObj(id, name) {
  this.id = id;
  this.name = name;
}

  
export const goods = [
    new GoodObj(0,'none','#FFFFFF'),
    new GoodObj(1,'Ivits','#FF0000'),
    new GoodObj(2,'Terran','#0000FF'),
    new GoodObj(3,'Xenon','#FFFF00'),
    new GoodObj(4,'Geoden','#F39c12'),
    new GoodObj(5,'Itars','#FFFFFF'),
    new GoodObj(6,'Firaks','#808B96'),
];


