import '@mantine/core/styles.css'
import React from 'react'
import { MantineProvider } from '@mantine/core'
import { Notifications } from '@mantine/notifications'
import { cssVariablesResolver, theme } from '@/theme'
import './global.css'

export const metadata = {
  title: 'Links',
  description: 'Prototype of a next iteration on search engines',
}

export default function RootLayout({ children }: { children: any }) {
  return (
    <html lang="en">
      <head>
        <link rel="shortcut icon" href="/favicon.svg" />
        <meta
          name="viewport"
          content="minimum-scale=1, initial-scale=1, width=device-width, user-scalable=no"
        />
      </head>
      <body>
        <MantineProvider
          defaultColorScheme="dark"
          cssVariablesResolver={cssVariablesResolver}
          theme={theme}
        >
          <Notifications />
          {children}
        </MantineProvider>
      </body>
    </html>
  )
}
