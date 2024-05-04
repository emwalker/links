'use client'

import { useState } from 'react'
import { Group, Title } from '@mantine/core'
import {
  IconUser,
  IconCircleLetterT,
  IconSwitchHorizontal,
  IconLogout,
} from '@tabler/icons-react'
import classes from './index.module.css'

const data = [
  { link: '', label: 'Users', icon: IconUser },
  { link: '', label: 'Topics', icon: IconCircleLetterT },
]

type Props = {
  title: string
}

export function Page({ title }: Props) {
  const [active, setActive] = useState('Billing')

  const links = data.map((item) => (
    <a
      className={classes.link}
      data-active={item.label === active || undefined}
      href={item.link}
      key={item.label}
      onClick={(event) => {
        event.preventDefault()
        setActive(item.label)
      }}
    >
      <item.icon className={classes.linkIcon} stroke={1.5} />
      <span>{item.label}</span>
    </a>
  ))

  return (
    <nav className={classes.navbar}>
      <div className={classes.navbarMain}>
        <Group className={classes.header} justify="space-between">
          <Title order={2}>{title}</Title>
        </Group>
        {links}
      </div>

      <div className={classes.footer}>
        <a href="#" className={classes.link} onClick={(event) => event.preventDefault()}>
          <IconSwitchHorizontal className={classes.linkIcon} stroke={1.5} />
          <span>Change account</span>
        </a>

        <a href="#" className={classes.link} onClick={(event) => event.preventDefault()}>
          <IconLogout className={classes.linkIcon} stroke={1.5} />
          <span>Logout</span>
        </a>
      </div>
    </nav>
  )
}
