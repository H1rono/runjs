console.log({ a: "b" });
console.error("Boom!");

const path = "./log.txt";
try {
    const contents = await runjs.readFile(path);
    console.log("Read from a file", contents);
} catch (e) {
    console.error(e);
}

await runjs.writeFile(path, "Lorem ipsum");
const contents = await runjs.readFile(path);
console.log("Read from file", path, "contents:", contents);
console.log("Remove file");
runjs.removeFile(path);
console.log("File removed");

console.log("Hello", "runjs!");
const content = await runjs.fetch(
    "https://deno.land/std@0.177.0/examples/welcome.ts",
);
console.log("Content from fetch", content);
