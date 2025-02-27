import { createContext, useState } from 'react';
import { Redirect } from "expo-router"
// import { client } from '../components/dynamicClient';
import { SessionProvider } from './session';
import '../global.css'

const Home = () => {

    return (
        <>
            {/* <client.reactNative.WebView /> */}
            {/* <Redirect href="/(auth)/sign-in"/> */}
            <Redirect href="/(root)/(tabs)/home"/>

        </>
    )
}

export default Home



    