<script setup>
import { useRoute, useRouter } from 'vue-router'
import Left  from './Left.vue'
import Right  from './Right.vue'
const router = useRouter()
const route = useRoute()


</script>

<template>
    <div class="workspace">
        <Left/>
        <Right/>
        <div class="clear"></div>
    </div>
</template>

<style scoped>
.workspace {
    width: 100%;
    height: 97vh;
}
.workspace > div {
    float: left;
}
.clear {
    clear: both;
}
</style>
