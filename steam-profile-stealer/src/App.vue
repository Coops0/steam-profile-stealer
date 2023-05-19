<script setup lang="ts">
import { useCookieStore } from "@/stores/cookie";
import CookieInput from "@/components/CookieInput.vue";

const cookieStore = useCookieStore();
const ws = new WebSocket('ws://localhost:8000/ws');

function send(object: any) {
  if (ws.readyState === ws.OPEN) {
    ws.send(JSON.stringify(object));
  }
}

ws.addEventListener('open', () => {
  console.log('websocket opened!');
  if(cookieStore.cookie) {
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
  const j = JSON.parse(data);
  console.log(j);

  console.log(Object.keys(j));
});
</script>


<template>
  <v-app>
    <v-main>
      <CookieInput v-if="!cookieStore.cookie" v-on:saveCookie="send({Cookie: {cookie: cookieStore.cookie}})"/>

    </v-main>
  </v-app>
</template>
