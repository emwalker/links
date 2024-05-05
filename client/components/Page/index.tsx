'use client'

import { useEffect, useState } from 'react'
import { Group, Title } from '@mantine/core'
import {
  IconUser,
  IconCircleLetterT,
  IconSwitchHorizontal,
  IconLogout,
  IconHome,
  IconSearch,
  IconBrandCodesandbox,
} from '@tabler/icons-react'
import { usePathname, useRouter } from 'next/navigation'
import Link from 'next/link'
import classes from './index.module.css'
import useSession from '@/lib/useSession'

const data = [
  { pathname: '/home', label: 'Home', icon: IconHome },
  { pathname: '/users', label: 'Users', icon: IconUser },
  { pathname: '/topics', label: 'Topics', icon: IconCircleLetterT },
  { pathname: '/search', label: 'Search', icon: IconSearch },
]

type Props = {
  children: any
}

export function Page({ children }: Props) {
  const pathname = usePathname()
  const [active, setActive] = useState(pathname)
  const { session, isLoading, logout } = useSession()
  const router = useRouter()

  useEffect(() => {
    if (!isLoading && !session.isLoggedIn) {
      router.replace('/login')
    }
  }, [isLoading, session.isLoggedIn, router])

  if (isLoading) {
    return <p className="text-lg">Loading...</p>
  }

  const links = data.map((item) => (
    <Link
      className={classes.link}
      data-active={item.pathname === active || undefined}
      href={item.pathname}
      key={item.label}
      onClick={() => {
        setActive(item.pathname)
      }}
    >
      <item.icon className={classes.linkIcon} stroke={1.5} />
      <span>{item.label}</span>
    </Link>
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
          <Link href="/change-account" className={classes.link}>
            <IconSwitchHorizontal className={classes.linkIcon} stroke={1.5} />
            <span>Change account</span>
          </Link>

          <Link
            href="/logout"
            className={classes.link}
            onClick={(event) => {
              event.preventDefault()
              logout()
            }}
          >
            <IconLogout className={classes.linkIcon} stroke={1.5} />
            <span>Logout</span>
          </Link>
        </div>
      </nav>

      <div className={classes.content}>
        {children}
      </div>
    </div>
  )
}
