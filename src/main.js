import init from "../pkg/InternetTry.js";
import {alert} from "./functions.js";


async function main() {
    await init();
    alert("Hi there Mr. White");
}
main();