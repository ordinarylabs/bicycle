# JavaScript SPROC Runtime for The Bicycle Project

Example SPROC script

```javascript
function main(message) {
    if (!(message instanceof Uint8Array)) {
        throw new Error("`message` will always be a Uint8Array");
    }

    // must always return a Uint8Array
    return new Uint8Array([1, 2, 3, 4]);
}
```