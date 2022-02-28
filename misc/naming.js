const vowel = ["a", "e", "i", "o", "u"];
const consonants = [
  "b",
  "c",
  "d",
  "f",
  "g",
  "h",
  "j",
  "k",
  "l",
  "m",
  "n",
  "p",
  "q",
  "r",
  "s",
  "t",
  "v",
  "w",
  "x",
  "y",
  "z",
];

const rand1 = new Date("1988-08-10 00:00:00").getTime();
const rand2 = 42;

const l1 = consonants[rand1 % (consonants.length - 1)]
const l2 = vowel[rand2 % (vowel.length - 1)]
const l3 = consonants[consonants.length - rand1 % consonants.length]

console.log(l1 + l2 + l3)