# Razor

## Descrição
Razor é a fundação modular e escalável para o desenvolvimento de jogos em Rust, projetada para ser o núcleo de frameworks e engines mais complexas. Este projeto oferece os sistemas essenciais para renderização 2D e 3D, gerenciamento de física, ECS (Entity Component System), input e áudio.

## Principais Funcionalidades
- **Renderização 2D/3D**: Sistema gráfico utilizando wgpu e ggez, permitindo renderizações eficientes e multi-plataforma.
- **Física 2D/3D**: Suporte a simulações físicas utilizando bibliotecas como rapier para oferecer colisão e movimentações realistas.
- **ECS (Entity Component System)**: Sistema para gerenciar entidades e seus componentes, facilitando o desenvolvimento de grandes sistemas.
- **Input**: Gerenciamento de eventos de teclado, mouse e gamepads utilizando a biblioteca winit.
- **Áudio**: Sistema para o gerenciamento de som e música utilizando rodio e outras ferramentas.
- **Modularidade Extrema**: Projete seus próprios sistemas ou adicione novos módulos de acordo com a necessidade.

## Estrutura do Projeto
- `/src`: Código fonte
  - `/core`: Componentes principais do sistema gráfico
    - `/graphics`: Tudo relacionado à renderização
      - `mod.rs`: Módulo principal de gráficos
      - `renderer2d.rs`: Implementação do renderizador 2D
      - `renderer3d.rs`: Implementação do renderizador 3D
    - `/physics`: Componentes principais do sistema de física
      - `mod.rs`: Módulo principal de física
      - `physics2d`: Implementação da fisica 2D
      - `physics3d`: Implementação da fisica 3D
  - `/ecs`: Componentes principais do sistema de entidade-componente
    - `mod.rs`: Módulo principal de ECS
    - `entity.rs`: Implementação das entidades
    - `component.rs`: Implementação dos componentes
    - `system.rs`: Implementação dos sistemas
  - `/input`: Componentes principais do sistema de input
    - `mod.rs`: Módulo principal de input
    - `keyboard.rs`: Implementação do gerenciamento de teclado
    - `mouse.rs`: Implementação do gerenciamento de mouse
  - `/audio`: Componentes principais do sistema de áudio
    - `mod.rs`: Módulo principal de áudio
    - `sound.rs`: Implementação do gerenciamento de som
    - `music.rs`: Implementação do gerenciamento de música

## Requisitos
- Rust
- Cargo

## Instalação
1. Clone o repositório:
   ```bash
   git clone [URL do repositório]
