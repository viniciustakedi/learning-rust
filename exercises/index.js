// const areaOrPerimeter = function (l, w) {
//   return l === w ?  l ** 2 :  (l * 2) + (w * 2)
// };

// console.log(areaOrPerimeter(6, 10));

const lcm = (...nums) => {
  const dividers = [];
  let currentDivider = 2;
  let currentNumbers = nums;

  const verifier = (arr) => {
    let response = true;
    for (value of arr) {
      if (value !== 1) {
        response = false;
        break;
      }
    }

    return response;
  }

  while (!verifier(currentNumbers)) {
    let nArr = [];
    let allIsOdd = false;

    for (cn of currentNumbers) {

    }
  }
};

console.log(lcm(2, 5))