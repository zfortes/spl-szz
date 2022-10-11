console.log("Iniciando a analise")

const res_spl = require('./result_spl.json')
const dataset = require('./bugfix_commits.json')
const res_raszz = require('./result_raszz.json')

// console.log(res_spl)

let not_founded = 0;
let right = 0;
res_spl.forEach(res => {
  if (res.founded_bug_commit_hash === " ") not_founded += 1;
  if (res.bug_commit_hash === res.founded_bug_commit_hash) right += 1;
})

const raszz_map = new Map();
const raszz_map_project = new Map();

res_raszz.forEach((bic) => {
  raszz_map.set(bic.founded_bug_commit_hash, bic)
  raszz_map_project.set(bic.repo_name, bic)
})

let raszz_right_count = 0;
let raszz_wrong_count = 0;
raszz_map.forEach((bic) => {
  const right = dataset.find((x) => x.fix_commit_hash === bic.fix_commit_hash);

  if (right.bug_commit_hash[0] === bic.founded_bug_commit_hash) {
    raszz_right_count += 1;
  } else {
    raszz_wrong_count += 1;
  }
})


console.log("tamanho total dataset", dataset.length)

console.log("RASZZ")
console.log("Total runned raszz = ", raszz_map_project.size)
console.log("raszz_right = ", raszz_right_count)
console.log("raszz_wrong= ", raszz_wrong_count)

console.log("___________________________________________________________________")
console.log("SPL-SZZ")
console.log("Right = ", right);

console.log("Not founded = ", not_founded)

console.log("Wrongs = ", res_spl.length - not_founded - right);

console.log("total ", res_spl.length)

