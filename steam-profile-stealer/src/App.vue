<script setup lang="ts">
import { useCookieStore } from "@/stores/cookie";
import CookieInput from "@/components/CookieInput.vue";
import type { Profile } from "@/stores/profile";

const cookieStore = useCookieStore();
const ws = new WebSocket('ws://localhost:8000/ws');

type SteamMessageIn =
    | { "StatusUpdate": { message: string } }
    | { "SelfProfile": { profile: Profile } }
    | { "ProfileFetch": { profile: Profile } }
    | { "Error": { message: string } }
    | { "NameChange": { name: string } }
    | { "PictureChange": { url: string } };

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

  const name = Object.keys(j)[0];
  switch (name) {
    case 'StatusUpdate':
      j['StatusUpdate'].message
      break;
    case 'SelfProfile':
      break;
    case 'ProfileFetch':
      break;
    case 'Error':
      break;
    case 'NameChange':
      break;
    case 'PictureChange':
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
