// Shadcn-inspired UI Components for Velocity Framework

import { createElement } from '../velocity-wasm-runtime';

// Button Component
export interface ButtonProps {
  children?: any;
  onClick?: (e: Event) => void;
  variant?: 'default' | 'destructive' | 'outline' | 'secondary' | 'ghost' | 'link';
  size?: 'default' | 'sm' | 'lg' | 'icon';
  className?: string;
  disabled?: boolean;
  type?: 'button' | 'submit' | 'reset';
}

export function Button(props: ButtonProps) {
  const {
    children,
    onClick,
    variant = 'default',
    size = 'default',
    className = '',
    disabled = false,
    type = 'button',
  } = props;

  const baseClasses = 'inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none';

  const variantClasses = {
    default: 'bg-primary text-primary-foreground hover:bg-primary/90',
    destructive: 'bg-destructive text-destructive-foreground hover:bg-destructive/90',
    outline: 'border border-input hover:bg-accent hover:text-accent-foreground',
    secondary: 'bg-secondary text-secondary-foreground hover:bg-secondary/80',
    ghost: 'hover:bg-accent hover:text-accent-foreground',
    link: 'underline-offset-4 hover:underline text-primary',
  };

  const sizeClasses = {
    default: 'h-10 py-2 px-4',
    sm: 'h-9 px-3 rounded-md',
    lg: 'h-11 px-8 rounded-md',
    icon: 'h-10 w-10',
  };

  return createElement(
    'button',
    {
      type,
      onClick,
      disabled,
      class: `${baseClasses} ${variantClasses[variant]} ${sizeClasses[size]} ${className}`,
    },
    children
  );
}

// Card Component
export interface CardProps {
  children?: any;
  className?: string;
}

export function Card(props: CardProps) {
  const { children, className = '' } = props;
  return createElement(
    'div',
    { class: `rounded-lg border bg-card text-card-foreground shadow-sm ${className}` },
    children
  );
}

export function CardHeader(props: CardProps) {
  const { children, className = '' } = props;
  return createElement(
    'div',
    { class: `flex flex-col space-y-1.5 p-6 ${className}` },
    children
  );
}

export function CardTitle(props: CardProps) {
  const { children, className = '' } = props;
  return createElement(
    'h3',
    { class: `text-2xl font-semibold leading-none tracking-tight ${className}` },
    children
  );
}

export function CardDescription(props: CardProps) {
  const { children, className = '' } = props;
  return createElement(
    'p',
    { class: `text-sm text-muted-foreground ${className}` },
    children
  );
}

export function CardContent(props: CardProps) {
  const { children, className = '' } = props;
  return createElement('div', { class: `p-6 pt-0 ${className}` }, children);
}

export function CardFooter(props: CardProps) {
  const { children, className = '' } = props;
  return createElement(
    'div',
    { class: `flex items-center p-6 pt-0 ${className}` },
    children
  );
}

// Input Component
export interface InputProps {
  type?: string;
  placeholder?: string;
  value?: any;
  onInput?: (e: Event) => void;
  onKeyPress?: (e: KeyboardEvent) => void;
  className?: string;
  disabled?: boolean;
}

export function Input(props: InputProps) {
  const {
    type = 'text',
    placeholder,
    value,
    onInput,
    onKeyPress,
    className = '',
    disabled = false,
  } = props;

  return createElement('input', {
    type,
    placeholder,
    value,
    onInput,
    onKeyPress,
    disabled,
    class: `flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 ${className}`,
  });
}

// Badge Component
export interface BadgeProps {
  children?: any;
  variant?: 'default' | 'secondary' | 'destructive' | 'outline';
  className?: string;
}

export function Badge(props: BadgeProps) {
  const { children, variant = 'default', className = '' } = props;

  const variantClasses = {
    default: 'bg-primary text-primary-foreground hover:bg-primary/80',
    secondary: 'bg-secondary text-secondary-foreground hover:bg-secondary/80',
    destructive: 'bg-destructive text-destructive-foreground hover:bg-destructive/80',
    outline: 'text-foreground border border-input',
  };

  return createElement(
    'div',
    {
      class: `inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 ${variantClasses[variant]} ${className}`,
    },
    children
  );
}

// Checkbox Component
export interface CheckboxProps {
  checked?: boolean;
  onChange?: (e: Event) => void;
  className?: string;
  disabled?: boolean;
}

export function Checkbox(props: CheckboxProps) {
  const { checked, onChange, className = '', disabled = false } = props;

  return createElement('input', {
    type: 'checkbox',
    checked,
    onChange,
    disabled,
    class: `peer h-4 w-4 shrink-0 rounded-sm border border-primary ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 ${className}`,
  });
}

// Separator Component
export interface SeparatorProps {
  orientation?: 'horizontal' | 'vertical';
  className?: string;
}

export function Separator(props: SeparatorProps) {
  const { orientation = 'horizontal', className = '' } = props;

  const orientationClasses = {
    horizontal: 'h-[1px] w-full',
    vertical: 'h-full w-[1px]',
  };

  return createElement('div', {
    class: `shrink-0 bg-border ${orientationClasses[orientation]} ${className}`,
  });
}
