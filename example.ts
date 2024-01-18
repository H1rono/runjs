interface Foo {
    bar: string;
    fizz: number;
}

let content: String = await runjs.fetch(
    "https://deno.land/std@0.177.0/examples/welcome.ts",
);
console.log("Content from fetch", content);
