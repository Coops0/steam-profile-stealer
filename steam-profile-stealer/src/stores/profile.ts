import { defineStore } from 'pinia'
import { ref } from "vue";

export interface Profile {
    name: string;
    image_url: string;
    url: string;
}

export const useProfileStore = defineStore('profile', () => {
    const selfProfile = ref<Profile | null>(null);
    const targetProfile = ref<Profile | null>(null);

    return {selfProfile, targetProfile};
});
