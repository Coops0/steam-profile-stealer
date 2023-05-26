<script setup lang="ts">
import { useCookieStore } from "@/stores/cookie";
import CookieInput from "@/components/CookieInput.vue";
import type { Profile } from "@/stores/profile";

const cookieStore = useCookieStore();
const ws = new WebSocket('ws://localhost:8000/ws');

type SteamMessageIn =
    | { tag: "status_update", fields: { message: string } }
    | { tag: "self_profile", fields: { profile: Profile } }
    | { tag: "profile_fetch", fields: { profile: Profile } }
    | { tag: "error", fields: { message: string } }
    | { tag: "name_change", fields: { name: string } }
    | { tag: "picture_change", fields: { url: string } };

type SteamMessageOut =
    | { "Cookie": { cookie: string } }
    | { "RefreshProfile": {} }
    | { "StealProfile": { name: string, image_url: string } }
    | { "FetchProfile": { url: string } };

function send(object: SteamMessageOut) {
  if (ws.readyState === ws.OPEN) {
    ws.send(JSON.stringify(object));
  }
}

ws.addEventListener('open', () => {
  console.log('websocket opened!');
  if (cookieStore.cookie) {
    send({Cookie: {cookie: cookieStore.cookie}});
  }
});

ws.addEventListener('close', c => {
  console.log('websocket closed', c);
});

ws.addEventListener('error', e => {
  console.error(e);
})

ws.addEventListener('message', ({data}) => {
  const j = JSON.parse(data) as SteamMessageIn;
  console.log(j);

  switch (j.tag) {
    case 'status_update':
      j.fields.message
      break;
    case 'self_profile':
      break;
    case 'profile_fetch':
      break;
    case 'error':
      break;
    case 'name_change':
      break;
    case 'picture_change':
      break;
  }
});
</script>


<template>
  <v-app>
    <v-main>
      <CookieInput v-if="!cookieStore.cookie" v-on:saveCookie="send({Cookie: {cookie: cookieStore.cookie}})"/>

    </v-main>
  </v-app>
</template>
