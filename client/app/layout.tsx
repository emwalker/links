import '@mantine/core/styles.css'
import React from 'react'
import { MantineProvider, ColorSchemeScript } from '@mantine/core'
import { Notifications } from '@mantine/notifications'
import { theme } from '../theme'

export const metadata = {
  title: 'Recommendations',
  description: 'Prototype of a next iteration on search engines',
}

export default function RootLayout({ children }: { children: any }) {
  return (
    <html lang="en">
      <head>
        <ColorSchemeScript />
        <link rel="shortcut icon" href="/favicon.svg" />
        <meta
          name="viewport"
          content="minimum-scale=1, initial-scale=1, width=device-width, user-scalable=no"
        />
      </head>
      <body>
        <MantineProvider
          defaultColorScheme="dark"
          theme={theme}
        >
          <Notifications />
          {children}
        </MantineProvider>
      </body>
    </html>
  )
}
