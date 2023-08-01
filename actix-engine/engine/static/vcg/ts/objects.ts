export interface GoodExt {
  id: number;
  name: string;
  color: Color;
}

export function createGoodObj(id: number, name: string, col_str: string): GoodExt {
  let color : Color = {str: col_str};
  return { id, name, color};
}

export interface PlayerExt {
  id: number;
  name: string;
}

export function createPlayerExt(id: number, name: string): PlayerExt {
  return { id, name };
}

export const NULL_GOOD: GoodExt = createGoodObj(6, 'none', '#FFFFFF');



export interface Color {
  str: string;
}



export interface Price {
  val: number;
}

export interface GoodWPriceExt {
  good: GoodExt;
  price: Price;
}

export interface OutputPairing {
  pl: PlayerExt;
  good_color_price: GoodWPriceExt | null;
}



export const goods: GoodExt[] = [
  createGoodObj(0, 'Firaks', '#808B96'),
  createGoodObj(1, 'Ivits', '#FF0000'),
  createGoodObj(2, 'Terran', '#0000FF'),
  createGoodObj(3, 'Xenon', '#FFFF00'),
  createGoodObj(4, 'Geoden', '#F39c12'),
  createGoodObj(5, 'Itars', '#FFFFFF'),
  NULL_GOOD,
];

export const NULL_GOOD_INDEX: number = goods.length - 1;
