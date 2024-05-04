'use client'

import { useState } from 'react'
import { Anchor, Group, Title } from '@mantine/core'
import {
  IconUser,
  IconCircleLetterT,
  IconSwitchHorizontal,
  IconLogout,
  IconHome,
  IconSearch,
  IconBrandCodesandbox,
} from '@tabler/icons-react'
import classes from './index.module.css'

const data = [
  { link: '/home', label: 'Home', icon: IconHome },
  { link: '/users', label: 'Users', icon: IconUser },
  { link: '/topics', label: 'Topics', icon: IconCircleLetterT },
  { link: '/search', label: 'Search', icon: IconSearch },
]

type Props = {
  children: any
}

export function Page({ children }: Props) {
  const [active, setActive] = useState('Home')

  const links = data.map((item) => (
    <Anchor
      className={classes.link}
      data-active={item.label === active || undefined}
      href={item.link}
      key={item.label}
      onClick={() => {
        // event.preventDefault()
        setActive(item.label)
      }}
    >
      <item.icon className={classes.linkIcon} stroke={1.5} />
      <span>{item.label}</span>
    </Anchor>
  ))

  return (
    <div className={classes.container}>
      <nav className={classes.navbar}>
        <div className={classes.navbarMain}>
          <Group className={classes.header} justify="left">
            <IconBrandCodesandbox stroke={1.5} />
            <Title order={3}>Recommendations</Title>
          </Group>
          {links}
        </div>

        <div className={classes.footer}>
          <Anchor href="/change-account" className={classes.link}>
            <IconSwitchHorizontal className={classes.linkIcon} stroke={1.5} />
            <span>Change account</span>
          </Anchor>

          <a href="#" className={classes.link} onClick={(event) => event.preventDefault()}>
            <IconLogout className={classes.linkIcon} stroke={1.5} />
            <span>Logout</span>
          </a>
        </div>
      </nav>

      <div className={classes.content}>
        {children}
      </div>
    </div>
  )
}
