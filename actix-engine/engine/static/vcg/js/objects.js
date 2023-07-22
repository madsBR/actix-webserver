"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
function createGoodObj(id, name, color) {
    return { id, name, color };
}
exports.createGoodObj = createGoodObj;
function createPlayerObj(id, name) {
    return { id, name };
}
exports.createPlayerObj = createPlayerObj;
exports.NULL_GOOD = createGoodObj(6, 'none', '#FFFFFF');
exports.goods = [
    createGoodObj(0, 'Firaks', '#808B96'),
    createGoodObj(1, 'Ivits', '#FF0000'),
    createGoodObj(2, 'Terran', '#0000FF'),
    createGoodObj(3, 'Xenon', '#FFFF00'),
    createGoodObj(4, 'Geoden', '#F39c12'),
    createGoodObj(5, 'Itars', '#FFFFFF'),
    exports.NULL_GOOD,
];
exports.NULL_GOOD_INDEX = exports.goods.length - 1;
