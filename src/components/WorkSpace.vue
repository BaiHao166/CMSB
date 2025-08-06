<script setup>
import { useRoute, useRouter } from 'vue-router'
import Left from './Left.vue'
import Right from './Right.vue'
import { ref } from 'vue'
const router = useRouter();
const route = useRoute();
let menuName = ref('');
let menu = ref('');
function clickMenu(menuName) {
    menu.value = menuName;
}

</script>

<template>
    <div class="workspace">
        <Left @click-once-menu="clickMenu" />
        <Right :menu="menu" />
        <div class="clear"></div>
    </div>
</template>

<style scoped>
.workspace {
    width: 100%;
    height: 97vh;
}

.workspace>div {
    float: left;
}

.clear {
    clear: both;
}
</style>
