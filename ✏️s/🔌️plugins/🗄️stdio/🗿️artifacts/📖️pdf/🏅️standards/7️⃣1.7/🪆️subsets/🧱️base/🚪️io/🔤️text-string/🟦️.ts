/** 🔤️ Canonical PDF text strings use BOM Unicode or Annex D.2 PDFDocEncoding. */
const low=[728,711,710,729,733,731,730,732];
const high=[8226,8224,8225,8230,8212,8211,402,8260,8249,8250,8722,8240,8222,8220,8221,8216,8217,8218,8482,64257,64258,321,338,352,376,381,305,322,339,353,382];
/** 📖️ Decodes the same admitted byte representation as the native PDF text-string owner. */
export function decodeTextString(source:readonly number[]):string{const bytes=Uint8Array.from(source);if(bytes[0]===254&&bytes[1]===255)return new TextDecoder("utf-16be").decode(bytes.subarray(2,2+Math.floor((bytes.length-2)/2)*2));if(bytes[0]===239&&bytes[1]===187&&bytes[2]===191)return new TextDecoder().decode(bytes.subarray(3));return Array.from(bytes,byte=>String.fromCodePoint(byte>=24&&byte<=31?low[byte-24]:byte>=128&&byte<=158?high[byte-128]:byte===160?8364:byte>=32&&byte<=126||byte>=161&&byte!==173?byte:65533)).join("");}
