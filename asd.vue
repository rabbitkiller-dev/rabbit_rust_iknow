<template>
<button @click="handleXXXX()"></button>
</template>
<script>

export default {
    methods: {
        validate() {

        },
        commit() {

        },
        handleXXXX() {
            if(!await this.validate()){
                return;
            }
            this.commit();
        }
    }
}
</script>

