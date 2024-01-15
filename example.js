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
