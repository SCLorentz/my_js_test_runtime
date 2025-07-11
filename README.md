# JS-runtime-test
 
## Manage windows

```javascript
let window = new Window("My Window").create();
```

## Custom protocol for console

```javascript
await console
    .clear()
    .log("hello world!\n")
    .input("what's your name? ")
    .then(name => console.log(`hello ${name}!`))
```