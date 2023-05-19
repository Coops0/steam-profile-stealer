import { defineStore } from 'pinia'
import { ref } from "vue";

export interface Profile {
    name: string;
    icon_url: string;
    url: string;
}

export const useProfileStore = defineStore('useProfileStore', () => {
    const selfProfile = ref<Profile | null>(null);
    const targetProfile = ref<Profile | null>(null);

    return {selfProfile, targetProfile};
});
