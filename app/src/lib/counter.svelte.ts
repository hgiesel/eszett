import { Counter } from "@hgiesel/eszett-client-web"

export function createCounter() {
    const counter = $derived(new Counter());
    const { value } = $derived(counter);

    return { 
        increment() {
            counter.increment()
            console.log(counter.value)
        },
        get value() {
            return value
        },
     };
}

export function createCounter2() {
    let value = $state(0);

    function increment() {
        value++
    }

    return { value, increment };
}