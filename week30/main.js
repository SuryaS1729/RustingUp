//same implementation in javascript
import { BSON, ObjectId } from "bson";

const data = { _id: new ObjectId(), name: "John", age: 30 };

const serialized = BSON.serialize(data);
const deserialized = BSON.deserialize(serialized);

console.log("Original Data:", data);
console.log("Serialized Data (BSON):", serialized);
console.log("Deserialized Data:", deserialized);
