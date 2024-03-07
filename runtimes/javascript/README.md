# JavaScript SPROC Runtime for The Bicycle Project

Example SPROC script

```javascript
// must always export a single `main()` function
function main(args) { // args is a deserialized JSON blob
    return { ...args }; // must always return an object that will be serialized to JSON
}
```