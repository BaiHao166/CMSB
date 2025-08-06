<script setup>
import UserInfo from './UserInfo.vue';
import Menus from './Menus.vue';
import { defineEmits } from "vue";

let emit = defineEmits(["clickOnceMenu"]);

function clickMenu(menuName) {
    emit("clickOnceMenu", menuName);
}

</script>

<template>
    <div class="left-area">
        <UserInfo></UserInfo>
        <Menus @click-once-menu="clickMenu"></Menus>
    </div>
</template>

<style scoped>
.left-area {
    width: 20%;
    height: 99.5%;
    background-color: white;
    border-radius: 10px;
    padding: 4px;
}
</style>