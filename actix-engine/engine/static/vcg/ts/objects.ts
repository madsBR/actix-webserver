export interface GoodObj {
  id: number;
  name: string;
  color: string;
}


export function createGoodObj(id: number, name: string, color: string): GoodObj {
  return { id, name, color };
}

export interface PlayerObj {
  id: number;
  name: string;
}

export function createPlayerObj(id: number, name: string): PlayerObj {
  return { id, name };
}

export const NULL_GOOD: GoodObj = createGoodObj(6, 'none', '#FFFFFF');

export const goods: GoodObj[] = [
  createGoodObj(0, 'Firaks', '#808B96'),
  createGoodObj(1, 'Ivits', '#FF0000'),
  createGoodObj(2, 'Terran', '#0000FF'),
  createGoodObj(3, 'Xenon', '#FFFF00'),
  createGoodObj(4, 'Geoden', '#F39c12'),
  createGoodObj(5, 'Itars', '#FFFFFF'),
  NULL_GOOD,
];

export const NULL_GOOD_INDEX: number = goods.length - 1;
