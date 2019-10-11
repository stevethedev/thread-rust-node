const path = require("path");
const ffi = require("ffi");

const library_name = path.resolve(__dirname, "../ffi/target/release/libffi");
const api = ffi.Library(library_name, {
  create_counters: ["int32", ["uint32", "uint32"]]
});

module.exports = {
  createCounters: api.create_counters
};
