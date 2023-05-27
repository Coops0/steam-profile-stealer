import type { Profile } from "@/stores/profile";
import { useMessageStore } from "@/stores/messages";
import { useCookieStore } from "@/stores/cookie";

export type SteamMessageIn =
    | { tag: "status_update", fields: { message: string } }
    | { tag: "self_profile", fields: { profile: Profile } }
    | { tag: "profile_fetch", fields: { profile: Profile } }
    | { tag: "error", fields: { message: string } }
    | { tag: "name_change", fields: { name: string } }
    | { tag: "picture_change", fields: { url: string } };

export type SteamMessageOut =
    | { tag: "cookie", fields: { cookie: string } }
    | { tag: "refresh_profile" }
    | { tag: "steal_profile", fields: { name: string, image_url: string } }
    | { tag: "fetch_profile", fields: { url: string } }


let ws: WebSocket | null = null;

const messageStore = useMessageStore();
const cookieStore = useCookieStore();

export function send(object: SteamMessageOut) {
    if (ws && ws.readyState === ws.OPEN) {
        ws.send(JSON.stringify(object));
    }
}

export function log(message: string, error: boolean = false) {
    messageStore.messages.unshift({message, key: Math.random().toString(), error});

    if (messageStore.messages.length > 70) {
        messageStore.messages.pop();
    }
}
export async function createWebsocket() {
    ws = new WebSocket('ws://localhost:8000/ws');

    ws.addEventListener('open', () => {
        log('Websocket successfully connected');
        console.log('websocket opened!');
        if (cookieStore.cookie) {
            send({tag: "cookie", fields: {cookie: cookieStore.cookie}});
        }
    });

    let retries = 0;

    ws.addEventListener('close', c => {
        console.log('websocket closed', retries, c);

        if (++retries > 5) {
            log('Gave up trying to reconnect after 6 retries, reload the page.', true);
        } else {
            log('Websocket disconnected, attempting to reconnect...', true);
            ws = new WebSocket('ws://localhost:8000/ws');
        }
    });

    ws.addEventListener('error', e => {
        console.error(e);

        log(`Websocket Error (logged to console): ${e}`, true)
    })
}