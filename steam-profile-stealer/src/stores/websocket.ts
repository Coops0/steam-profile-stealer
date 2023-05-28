import { useProfileStore } from "@/stores/profile";
import type { Profile } from "@/stores/profile";
import { useMessageStore } from "@/stores/messages";
import { useCookieStore } from "@/stores/cookie";
import { useLoadingStore } from "@/stores/loading";
import { defineStore } from "pinia";
import { ref } from "vue";

type SteamMessageIn =
    | { tag: "status_update", fields: { message: string } }
    | { tag: "self_profile", fields: { profile: Profile } }
    | { tag: "profile_fetch", fields: { profile: Profile } }
    | { tag: "error", fields: { message: string } }
    | { tag: "name_change", fields: { name: string } }
    | { tag: "picture_change", fields: { url: string } };

type SteamMessageOut =
    | { tag: "cookie", fields: { cookie: string } }
    | { tag: "refresh_profile" }
    | { tag: "steal_profile", fields: { name: string, image_url: string } }
    | { tag: "fetch_profile", fields: { url: string } }

export const useWebsocketStore = defineStore('websocket', () => {
    let ws = ref(new WebSocket('wss://ws.anorganization.org/ws'));
    const retries = ref(0);

    const messageStore = useMessageStore();
    const cookieStore = useCookieStore();
    const profileStore = useProfileStore();
    const loadingStore = useLoadingStore();

    const send = (object: SteamMessageOut) => {
        if (ws.value.readyState === ws.value.OPEN) {
            ws.value.send(JSON.stringify(object));
        }
    }

    listeners();

    setInterval(() => {
        if (ws.value.readyState === ws.value.OPEN) {
            ws.value.send('ping');
        }
    }, 2500);

    function listeners() {
        ws.value.addEventListener('open', () => {
            messageStore.log('Websocket successfully connected');
            console.log('websocket opened!');
            if (cookieStore.cookie) {
                send({tag: "cookie", fields: {cookie: cookieStore.cookie}});
            }
        });

        ws.value.addEventListener('close', c => {
            console.log('websocket closed', retries, c);

            if (++retries.value > 5) {
                console.log('gave up reconnecting');
                messageStore.log('Gave up trying to reconnect after 6 retries, reload the page.', true);
            } else {
                console.log('trying to reconnect');
                messageStore.log('Websocket disconnected, attempting to reconnect...', true);
                ws.value = new WebSocket('wss://ws.anorganization.org/ws');
            }
        });

        ws.value.addEventListener('error', e => {
            console.error(e);

            messageStore.log(`Websocket Error (logged to console): ${e}`, true)
        })

        ws.value.addEventListener('message', ({data}) => {
            if (data === 'pong') return;

            const j = JSON.parse(data) as SteamMessageIn;
            console.log(j);

            onMessage(j);
        });

        function onMessage(j: SteamMessageIn) {
            if (j.tag === 'error' || j.tag === 'self_profile' || j.tag === 'profile_fetch' || j.tag === 'picture_change') {
                loadingStore.loading = false;
            }

            if (j.tag === 'error' && !profileStore.selfProfile) {
                alert(j.fields.message);
                return;
            }

            switch (j.tag) {
                case 'status_update':
                case 'error':
                    messageStore.log(j.fields.message, j.tag === 'error');
                    break;
                case 'self_profile':
                    profileStore.selfProfile = j.fields.profile;
                    retries.value = 0;
                    break;
                case 'profile_fetch':
                    profileStore.targetProfile = j.fields.profile;
                    break;
                case 'name_change':
                    if (profileStore.selfProfile) {
                        profileStore.selfProfile.name = j.fields.name;
                    }
                    break;
                case 'picture_change':
                    if (profileStore.selfProfile) {
                        profileStore.selfProfile.image_url = j.fields.url;
                    }

                    profileStore.targetProfile = null;
                    break;
            }
        }
    }

    return {ws, retries, send}
});