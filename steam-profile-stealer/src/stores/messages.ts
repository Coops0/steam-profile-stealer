import { defineStore } from "pinia";
import { reactive } from "vue";

export const useMessageStore = defineStore('message', () => {
    const messages = reactive<{message: string, key: string, error: boolean}[]>([]);

    const log = (message: string, error: boolean = false) => {
        messages.unshift({message, key: Math.random().toString(), error});

        if (messages.length > 70) {
            messages.pop();
        }
    }

    return {messages, log};
});