import { defineStore } from "pinia";
import { ref } from "vue";

export const useCookieStore = defineStore('cookie', () => {
    const cookie = ref<string | null>(null);

    return {cookie};
});