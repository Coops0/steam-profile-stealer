import { defineStore } from "pinia";
import { ref } from "vue";

export const useMessageStore = defineStore('message', () => {
    const messages = ref<{message: string, key: string, error: boolean}[]>([]);

    return {messages};
});